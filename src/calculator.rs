#[derive(Default)]
pub struct Core {
    num1: u32,
    op: Option<Operator>,
    num2: Option<u32>,
}
impl Core {
    pub fn number(&mut self, num: u8) {
        let num = num as u32;
        if self.op.is_some() {
            self.num2 = Some(self.num2.unwrap_or(0) * 10 + num)
        } else {
            self.num1 = self.num1 * 10 + num
        }
    }
    pub fn operator<O: Into<Operator>>(&mut self, op: O) {
        let operator = op.into();
        if self.num2.is_some() {
            self.calculate();
        }
        self.op = Some(operator)
    }
    pub fn calculate(&mut self) -> Option<()> {
        let op = self.op.clone()?;
        let num2 = self.num2.clone()?;
        self.num1 = match op {
            Operator::Add => self.num1 + num2,
            Operator::Sub => self.num1 - num2,
            Operator::Mul => self.num1 * num2,
            Operator::Div => self.num1 / num2,
        };
        self.num2 = None;
        self.op = None;
        Some(())
    }
    pub fn view(&self) -> String {
        let mut view = self.num1.to_string();
        if let Some(ref op) = self.op {
            let op = match op {
                Operator::Add => " + ",
                Operator::Sub => " - ",
                Operator::Mul => " * ",
                Operator::Div => " / ",
            };
            view.push_str(op);
        }
        if let Some(num) = self.num2 {
            view.push_str(&num.to_string())
        }
        view
    }
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}
impl From<u8> for Operator {
    // 0 => Add
    // 1 => Sub
    // 2 => Mul
    // 3 => Div
    fn from(o: u8) -> Self {
        match o {
            1 => Operator::Sub,
            2 => Operator::Mul,
            3 => Operator::Div,
            _ => Operator::Add,
        }
    }
}
