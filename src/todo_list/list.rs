use anyhow::Result;

pub trait TodoList {
	fn new_task(&self, content: String) -> Result<()>;

	fn get_tasks(&self) -> Result<String>;

	fn change_task(&self, id: usize, content: String) -> Result<()>;
	fn mark_completed(&self, id: usize) -> Result<()>;

	fn remove_task(&self, id: usize) -> Result<()>;
}
