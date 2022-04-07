use crate::beano::Beano;
use crate::common::Point;
use std::collections::HashMap;
pub struct Beanoz {
    inner: HashMap<Point<usize>, (Beano, usize)>,
}

impl Beanoz {
    pub fn get(&mut self, pos: &Point<usize>) -> Option<&mut Beano> {
        Some(&mut self.inner.get_mut(pos)?.0)
    }

    pub fn remove(&mut self, pos: &Point<usize>) {
        self.inner.remove(pos);
    }

    pub fn update(&mut self, beano: &Beano, new_pos: &Point<usize>) {
        if let Some((pt, f)) = self.inner.iter().find(|(_, x)| x.0 == beano) {
            self.inner.remove(pt);
            self.inner.insert(new_pos.clone(), f.to_owned());
        }
    }

    pub fn reset(&mut self) {
        for key in self.inner.keys() {
            self.inner.get_mut(key).unwrap().0.reset();
        }
    }

    pub fn next_pos(&self) -> Option<Point<usize>> {
        Some(
            self.inner
                .iter()
                .filter(|(_, (beano, _))| !beano.done_action)
                .next()?
                .0
                .to_owned(),
        )
    }
}

impl From<Vec<(Point<usize>, Beano)>> for Beanoz {
    fn from(inner: Vec<(Point<usize>, Beano)>) -> Self {
        Self {
            inner: inner
                .into_iter()
                .enumerate()
                .map(|(index, (pos, beano))| (pos, (beano, index)))
                .collect(),
        }
    }
}

impl Into<Vec<(Point<usize>, Beano)>> for Beanoz {
    fn into(self) -> Vec<(Point<usize>, Beano)> {
        let mut k = self
            .inner
            .into_iter()
            .map(|(pos, (beano, index))| (index, pos, beano))
            .collect::<Vec<_>>();

        k.sort_by_key(|(i, _, _)| i);

        k.into_iter().map(|(_, pos, beano)| (pos, beano)).collect()
    }
}
