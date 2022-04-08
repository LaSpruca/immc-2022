use crate::beano::Beano;
use crate::common::Point;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Beanoz {
    inner: HashMap<Point<usize>, (Beano, usize)>,
}

impl Beanoz {
    pub fn insert(&mut self, point: &Point<usize>, beano: Beano) {
        let count = self.inner.len();
        self.inner.insert(*point, (beano, count));
    }
}

impl Beanoz {
    pub fn get(&mut self, pos: &Point<usize>) -> Option<&mut Beano> {
        Some(&mut self.inner.get_mut(pos)?.0)
    }

    pub fn remove(&mut self, pos: &Point<usize>) {
        self.inner.remove(pos);
    }

    pub fn update(&mut self, old_pos: &Point<usize>, new_pos: &Point<usize>) {
        if let Some(o) = self.inner.remove(old_pos) {
            self.inner.insert(*new_pos, o);
        }
    }

    pub fn reset(&mut self) {
        for key in self.inner.clone().keys() {
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

    pub fn all_seated(&self) -> bool {
        self.inner.len() > 0
            && self
                .inner
                .iter()
                .filter(|(_, (beano, _))| !beano.is_seated())
                .count()
                == 0
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

        k.sort_by_key(|(i, _, _)| *i);

        k.into_iter().map(|(_, pos, beano)| (pos, beano)).collect()
    }
}
