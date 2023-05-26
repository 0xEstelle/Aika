pub enum ProcessType {
    Deterministic,
    Stochastic,
}

pub enum ProcessDuration {
    Finite,
    Infinite,
}

pub struct StandardProcess {
    pub process_id: u16,
    pub process_name: String,
    pub process_type: ProcessType::Deterministic,
    pub process_duration: ProcessDuration::Infinite,
    pub process: fn(),
}

impl StandardProcess {
    pub fn add_new_event(&mut self, env: &mut Environment) {
        let process = Box::new(Self {
            process_id: self.process_id,
            process_name: self.process_name,
            process_type: self.process_type,
            process_duration: self.process_duration,
            process: self.process,
        });
        let new_event = Event {
            time: env.curr_event + 1u64,
            process: RefCell::new(process),
        };
        env.add_event(new_event);
    }
}

impl Process for StandardProcess {
    fn run(&mut self, env: &mut Environment) {
        self.process();
        self.add_new_event(env);
    }
}


pub struct FiniteProcess {
    pub process_id: u16,
    pub process_name: String,
    pub process_type: ProcessType::Deterministic,
    pub process_duration: ProcessDuration::Finite,
    pub process: fn(),
    pub process_end_event: u64,
}

impl FiniteProcess {
    pub fn add_new_event(&mut self, env: &mut Environment) {
        let process = Box::new(Self {
            process_id: self.process_id,
            process_name: self.process_name,
            process_type: self.process_type,
            process_duration: self.process_duration,
            process: self.process,
            process_end_event: self.process_end_event,
        });
        let new_event = Event {
            time: env.curr_event + 1u64,
            process: RefCell::new(process),
        };
        if env.curr_event < self.process_end_event {
            env.add_event(new_event);
        } else {
            break;
        }
    }
}

impl Process for FiniteProcess {
    fn run(&mut self, env: &mut Environment) {
        self.process();
        if env.curr_event < self.process_end_time {
            self.add_new_event(env);
        } else {
            break;
        }
    }
}
