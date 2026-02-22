use crate::prelude::*;
use super::EnvironmentTiles;

impl EnvironmentTiles {
    pub fn inner_top_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_top_left,
        }
    }

    pub fn inner_top_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_top_center,
        }
    }

    pub fn inner_top_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_top_right,
        }
    }

    pub fn inner_center_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_center_left,
        }
    }

    pub fn inner_center_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_center_center,
        }
    }

    pub fn inner_center_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_center_right,
        }
    }

    pub fn inner_bottom_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_bottom_left,
        }
    }

    pub fn inner_bottom_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_bottom_center,
        }
    }

    pub fn inner_bottom_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.inner_bottom_right,
        }
    }

    pub fn outer_top_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_top_left,
        }
    }

    pub fn outer_top_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_top_center,
        }
    }

    pub fn outer_top_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_top_right,
        }
    }

    pub fn outer_center_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_center_left,
        }
    }

    pub fn outer_center_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_center_center,
        }
    }

    pub fn outer_center_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_center_right,
        }
    }

    pub fn outer_bottom_left(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_bottom_left,
        }
    }

    pub fn outer_bottom_center(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_bottom_center,
        }
    }

    pub fn outer_bottom_right(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: self.outer_bottom_right,
        }
    }
}
