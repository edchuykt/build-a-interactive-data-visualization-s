Rust
// Configuration file for interactive data visualization simulator

// Import necessary crates
extern crate piston2d_opengl_graphics;
extern crate pistoncore_event;
extern crate rand;

// Simulator configuration
struct SimulatorConfig {
    // Window dimensions
    width: u32,
    height: u32,

    // Data source (e.g. CSV file, database connection)
    data_source: String,

    // Visualization type (e.g. scatter plot, bar chart)
    vis_type: String,

    // Interactive features (e.g. zoom, hover)
    interactive_features: Vec<String>,
}

// Data point struct
struct DataPoint {
    x: f64,
    y: f64,
    label: String,
}

// Simulator state
struct SimulatorState {
    config: SimulatorConfig,
    data_points: Vec<DataPoint>,
    current_vis: String,
    zoom_level: f64,
    hover_point: Option<DataPoint>,
}

// Event handlers
fn handle_mouse_move(state: &mut SimulatorState, x: f64, y: f64) {
    // Update hover point
    state.hover_point = state.data_points.iter().find(|dp| {
        dp.x - 10.0 <= x && dp.x + 10.0 >= x && dp.y - 10.0 <= y && dp.y + 10.0 >= y
    }).cloned();
}

fn handle_mouse_wheel(state: &mut SimulatorState, delta: f64) {
    // Update zoom level
    state.zoom_level += delta;
}

fn main() {
    // Initialize simulator config
    let mut config = SimulatorConfig {
        width: 800,
        height: 600,
        data_source: "data.csv".to_string(),
        vis_type: "scatter plot".to_string(),
        interactive_features: vec!["zoom".to_string(), "hover".to_string()],
    };

    // Initialize simulator state
    let mut state = SimulatorState {
        config,
        data_points: vec![],
        current_vis: "scatter plot".to_string(),
        zoom_level: 1.0,
        hover_point: None,
    };

    // Load data from data source
    state.data_points = load_data_from_source(&state.config.data_source);

    // Initialize OpenGL graphics
    let opengl = piston2d_opengl_graphics::OpenGL::new(state.config.width, state.config.height);

    // Set up event handlers
    let mut events = pistoncore_event::Events::new();
    events.on_mouse_move(handle_mouse_move);
    events.on_mouse_wheel(handle_mouse_wheel);

    // Run simulator
    loop {
        // Clear screen
        opengl.clear([1.0; 4]);

        // Draw visualization
        draw_visualization(&state, &opengl);

        // Update display
        opengl.flip();
    }
}

// Load data from data source (e.g. CSV file, database connection)
fn load_data_from_source(source: &str) -> Vec<DataPoint> {
    // TO DO: implement data loading logic
    vec![]
}

// Draw visualization (e.g. scatter plot, bar chart)
fn draw_visualization(state: &SimulatorState, opengl: &piston2d_opengl_graphics::OpenGL) {
    // TO DO: implement visualization drawing logic
}