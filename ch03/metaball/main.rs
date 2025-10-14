mod app;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut sample_count = 1u32;
    let mut resolution = 96u32;
    let mut colormap_name = "jet";

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        sample_count = args[1].parse::<u32>().unwrap();
    }
    if args.len() > 2 {
        resolution = args[2].parse::<u32>().unwrap();
    }
    if args.len() > 3 {
        colormap_name = &args[3];
    }
    
    let title = "ch03 metaball";

    let _ = run(sample_count, resolution, colormap_name, title);

    pub fn run(
        sample_count: u32,
        resolution: u32,
        colormap_name: &str,
        title: &'static str,
    ) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::with_user_event().build()?;
        let mut app = App::new(sample_count, resolution, colormap_name, title);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}