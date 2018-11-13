use interface::*;
use calculator::Core;

pub struct Calculator {
    emit: CalculatorEmitter,
    view: String,
    core: Core,
}
impl Calculator {
    fn update_view(&mut self) {
        self.view = self.core.view();
        self.emit.view_changed()
    }
}
impl CalculatorTrait for Calculator {
    fn new(emit: CalculatorEmitter) -> Self {
        let core = Core::default();
        Calculator {
            emit: emit,
            view: core.view(),
            core,
        }
    }
    fn emit(&mut self) -> &mut CalculatorEmitter {
        &mut self.emit
    }
    fn view(&self) -> &str {
        &self.view
    }
    fn number(&mut self, num: u8) {
        self.core.number(num);
        self.update_view()
    }
    fn op(&mut self, op: u8) {
        self.core.operator(op);
        self.update_view()
    }
    fn calculate(&mut self) {
        if self.core.calculate().is_some() {
            self.update_view()
        }
    }
    fn clear(&mut self) {
        self.core = Core::default();
        self.update_view()
    }
}

