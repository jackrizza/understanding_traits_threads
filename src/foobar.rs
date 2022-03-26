use std::sync::{Arc, Mutex};


#[derive(Debug, Copy, Clone)]
pub struct Foobar<FOO, BAR> {
    pub foo : FOO,
    pub bar : BAR,
}


impl<FOO, BAR> Foobar<FOO, BAR> 
where FOO : PartialEq {
    pub fn new(foo : FOO, bar : BAR) -> Self {
        Foobar {
            foo : foo,
            bar : bar,
        }
    }


    pub fn set(&mut self, foo : FOO, new_bar : BAR) {
        if self.foo == foo {
            self.bar = new_bar;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Incasement<FOO, BAR>(Arc<Mutex<Vec<Foobar<FOO, BAR>>>>);



impl<FOO, BAR> Incasement<FOO, BAR> 
where FOO : PartialEq + Clone,
      BAR : Clone {
    
    pub fn new(genisis : Foobar<FOO, BAR>) -> Self {
        Incasement(Arc::new(Mutex::new(vec![genisis])))
    }

    pub fn list(&self) -> Vec<Foobar<FOO,BAR>> {
        let list = self.0.lock().unwrap();
        list.clone()
    }

    pub fn push(&mut self, foobar : Foobar<FOO, BAR>) {
        let mut vec_fb = self.0.lock().unwrap();
        vec_fb.push(foobar)
    }
    
    pub fn lookup(&mut self, foo : FOO) -> usize  {
        let block = self.0.lock().unwrap();
        block.iter().position(|r| r.foo == foo).unwrap()
    }

    pub fn update(&mut self, foo : FOO, new_bar : BAR) {
        let i = self.lookup(foo.clone());
        self.0.lock().unwrap()[i].set(foo, new_bar)
    }
}


