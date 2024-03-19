use std::thread::JoinHandle;
use wgpu::util::DeviceExt;

fn main() {
    let task = std::sync::Arc::new(std::sync::Mutex::new(std::collections::vec_deque::VecDeque::new()));
    let task_clone = task.clone();
    let thread_console_handle: JoinHandle<_> = std::thread::spawn(|| {
        thread_console(task_clone);
    });

    let task_clone = task.clone();
    let thread_gpu_handle: JoinHandle<_> = std::thread::spawn(|| {
        thread_gpu(task_clone);
    });

    thread_console_handle.join().unwrap();
    thread_gpu_handle.join().unwrap();
}

fn thread_console(task: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<String>>>) {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    while !buffer.eq("exit") {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        buffer.remove(buffer.len() - 1);
        let buffer_temp = buffer.clone();

        let args = buffer_temp.split(' ').collect::<Vec<&str>>();
        match args[0] {
            "create" => {
                let name = String::from(args[1]);
                let rows = String::from(args[2]);
                let cols = String::from(args[3]);
                let mut task = task.lock().unwrap();
                task.push_back(format!("create {} {} {}", name, rows, cols));
            },
            "delete" => {
                let mut task = task.lock().unwrap();
                task.push_back(format!("delete {}", args[1]));
            },
            "print" => {
                let mut task = task.lock().unwrap();
                task.push_back(format!("print {}", args[1]));
            },
            "multiply" => {
                let mut task = task.lock().unwrap();
                task.push_back(format!("multiply {} {} {}", args[1], args[2], args[3]));
            },
            "set" => {
                let mut task = task.lock().unwrap();
                task.push_back(format!("set {} {} {} {}", args[1], args[2], args[3], args[4]));
            },
            "exit" => {
                let mut task = task.lock().unwrap();
                task.push_back(String::from("exit"));
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
}

fn create_input_buffer(device: &wgpu::Device, data: &Vec<f32>) -> wgpu::Buffer {
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        usage: wgpu::BufferUsages::STORAGE,
        contents: bytemuck::cast_slice(&data),
    });
    input_buffer
}

fn create_scalar_buffer(device: &wgpu::Device, data: u32) -> wgpu::Buffer {
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        usage: wgpu::BufferUsages::STORAGE,
        contents: bytemuck::cast_slice(&[data]),
    });
    input_buffer
}

fn create_output_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: size as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    output_buffer
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    bind_group_layout
}

fn create_bind_group(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout,
                     input_buffer1: &wgpu::Buffer, input_buffer2: &wgpu::Buffer,
                     output_buffer: &wgpu::Buffer, k_max_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &input_buffer1,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &input_buffer2,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &output_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &k_max_buffer,
                    offset: 0,
                    size: None,
                }),
            }
        ],
    });
    bind_group
}

struct WgpuStruct {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_shader: wgpu::ShaderModule,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::ComputePipeline,
}

async fn init_wgpu() -> WgpuStruct {
    let inst: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = inst.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }).await.unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

    let compute_shader = include_str!("../shaders/shader.wgsl");

    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(compute_shader.into()),
    });

    let bind_group_layout = create_bind_group_layout(&device);
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    });
    let wgpu_struct = WgpuStruct {
        device,
        queue,
        compute_shader: cs_module,
        bind_group_layout,
        pipeline_layout,
        pipeline,
    };
    wgpu_struct
}

fn create_compute_pass(encoder: &mut wgpu::CommandEncoder, pipeline: &wgpu::ComputePipeline, bind_group: &wgpu::BindGroup, i: u32, j: u32) {
    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute Pass"), timestamp_writes: None });
    compute_pass.set_pipeline(&pipeline);
    compute_pass.set_bind_group(0, &bind_group, &[]);
    compute_pass.dispatch_workgroups(i, j, 1);
}

fn thread_gpu(task: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<String>>>) {
    let mut matrixes: std::collections::HashMap<String, (Vec<f32>, usize, usize)> = std::collections::HashMap::new();

    let wgpu_struct = futures::executor::block_on(init_wgpu());

    loop {
        let message: String;
        {
            let mut task = task.lock().unwrap();
            if task.is_empty() {
                continue;
            }
            message = task.pop_front().unwrap();
        }
        let args = message.split(' ').collect::<Vec<&str>>();
        match args[0] {
            "create" => {
                let name = String::from(args[1]);
                let rows = args[2].parse::<usize>().unwrap();
                let cols = args[3].parse::<usize>().unwrap();
                let data = vec![0.0; rows * cols];
                matrixes.insert(name, (data, rows, cols));
            },
            "delete" => {
                let name = String::from(args[1]);
                matrixes.remove(&name);
            },
            "print" => {
                let name = String::from(args[1]);
                let matrix = matrixes.get(&name).unwrap();
                for i in 0..matrix.1 {
                    for j in 0..matrix.2 {
                        print!("{} ", matrix.0[i * matrix.2 + j]);
                    }
                    println!();
                }
            },
            "multiply" => {
                let name1 = String::from(args[1]);
                let name2 = String::from(args[2]);
                let matrix1 = matrixes.get(&name1).unwrap();
                let matrix2 = matrixes.get(&name2).unwrap();

                let mut encoder = wgpu_struct.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder") });

                let input_buffer1 = create_input_buffer(&wgpu_struct.device, &matrix1.0);
                let input_buffer2 = create_input_buffer(&wgpu_struct.device, &matrix2.0);
                let k_max_buffer = create_scalar_buffer(&wgpu_struct.device, matrix1.2 as u32);
                let output_buffer = create_output_buffer(&wgpu_struct.device, matrix1.1 * matrix2.2 * std::mem::size_of::<f32>());
                let output_intermediary_buffer = wgpu_struct.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Map Buffer 2"),
                    size: (matrix1.1 * matrix2.2 * std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                let bind_group = create_bind_group(&wgpu_struct.device, &wgpu_struct.bind_group_layout,
                                                   &input_buffer1, &input_buffer2,
                                                   &output_buffer, &k_max_buffer);

                create_compute_pass(&mut encoder, &wgpu_struct.pipeline, &bind_group, matrix1.1 as u32, matrix2.2 as u32);

                encoder.copy_buffer_to_buffer(&output_buffer, 0, &output_intermediary_buffer, 0, output_buffer.size());

                wgpu_struct.queue.submit(Some(encoder.finish()));
                wgpu_struct.device.poll(wgpu::Maintain::Wait);

                let result_vector: Vec<f32>;

                {
                    let buffer_slice = output_intermediary_buffer.slice(..);
                    buffer_slice.map_async(wgpu::MapMode::Read, move |_result| {});
                    wgpu_struct.device.poll(wgpu::Maintain::Wait);

                    let data = buffer_slice.get_mapped_range();
                    result_vector = bytemuck::cast_slice(&data).to_vec();
                }
                matrixes.insert(String::from(args[3]), (result_vector, matrix1.1, matrix2.2));
            },
            "set" => {
                let name = String::from(args[1]);
                let row = args[2].parse::<usize>().unwrap();
                let col = args[3].parse::<usize>().unwrap();
                let value = args[4].parse::<f32>().unwrap();
                let matrix = matrixes.get_mut(&name).unwrap();
                matrix.0[row * matrix.2 + col] = value;
            },
            "exit" => {
                break;
            }
            _ => {}
        }
    }
}

