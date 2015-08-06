// pub enum Tag {
// 	tag_0,
// }

// pub trait Script;
// struct LogicScript: Script;
// struct LogicScript2: Script;

// pub struct SampleObject { // GameObject
// 	pos: Position2D, // Position & Rotation & Vertex
// 	name: String,
// 	tag: Tag,
// 	logic_script: LogicScript, // Attached MonoBehaviour
// 	logic_script2: LogicScript2,
// 	enabled: bool
// }
// // each object - components : has-a relation

// impl SampleObject {
// 	pub fn get_component<T: Script>(&mut self) -> Option<&mut T> {

// 	} // make it macro

// 	pub fn set_active(&mut self, enabled: bool) {
// 		self.enabled = enabled;
// 	}
// }

// impl Object for SampleObject {
// 	fn update(&mut self, elapsed: f32) {

// 	}

// 	fn draw(&self, target: &mut Frame, camera: Matrix) -> Result <(), DrawError> {

// 	}
// }