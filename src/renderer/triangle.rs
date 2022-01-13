use super::vertex::{Vertex2D, Vertex3D};

pub struct BoundingSquare<T: Copy + std::cmp::PartialEq + std::fmt::Debug + 'static> {
    top_left: Vertex2D<T>,
    top_right: Vertex2D<T>,
    bottom_left: Vertex2D<T>,
    bottom_right: Vertex2D<T>,
}

pub struct Triangle2D<T: Copy + std::cmp::PartialEq + std::fmt::Debug + 'static> {
    vertices: [Vertex2D<T>; 3]
}

impl<T: Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + 'static> Triangle2D<T> {
    fn is_back_facing(&self) -> bool {
        false
    }
    
    fn get_bounding_square(&self) -> BoundingSquare<T> {
        let v1 = unsafe { self.vertices.get_unchecked(1) };
        let v2 = unsafe { self.vertices.get_unchecked(2) };
        let v3 = unsafe { self.vertices.get_unchecked(3) };
        
        let (mut y_min, mut y_max) = if v1.y < v2.y {
            (v1.y, v2.y)
        } else {
            (v2.y, v1.y)
        };
        if y_min > v3.y {
            y_min = v3.y;
        } else if y_max < v3.y {
            y_max = v3.y;
        }

        let (mut x_min, mut x_max) = if v1.x < v2.x {
            (v1.x, v2.x)
        } else {
            (v2.x, v1.x)
        };
        if x_min > v3.x {
            x_min = v3.x;
        } else if x_max < v3.x {
            x_max = v3.x;
        }

        BoundingSquare {
            bottom_left: Vertex2D::new(x_min, y_min),
            bottom_right: Vertex2D::new(x_max, y_min),
            top_left: Vertex2D::new(x_min, y_max),
            top_right: Vertex2D::new(x_max, y_max)
        }
    }
}

pub struct Triangle3D<T: Copy + std::cmp::PartialEq + std::fmt::Debug + 'static> {
    vertices: Vertex3D<T>
}
