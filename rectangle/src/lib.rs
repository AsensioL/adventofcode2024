use std::fmt;
use std::error::Error;


// ------------ RectangleError class ------------
#[derive(PartialEq, Debug)]
pub enum RectangleError {
    DifferentWidths,
    InvalidCharacter(usize, usize, char)
}

impl fmt::Display for RectangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RectangleError::DifferentWidths => write!(f, "Rows have different widths."),
            RectangleError::InvalidCharacter(row,col, ch) => write!(f, "Invalid character `{ch}`found while parsing at row {row}, column {col}."),
        }
    }
}
impl Error for RectangleError {}


// ------------ Rectangle class ------------
#[derive(Debug)]
pub struct Rectangle<T> {
    pub data: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize
}
type RectangleResult<T> = Result<Rectangle<T>, RectangleError>;

impl<T> Rectangle<T> {
    pub fn from_vec(data: Vec<Vec<T>>) -> RectangleResult<T> {
        // Height
        let height = data.len();

        // Width
        let width = {
            // Width of the first column
            let width_first = data.first().map(|first_row| first_row.len() ).unwrap_or(0);

            // All rows must have the same width
            if data.iter().all( |row| row.len() == width_first ) {
                width_first
            }
            else {
                return Err(RectangleError::DifferentWidths)
            }
        };

        Ok( Rectangle::<T> {data, height, width} )
    }

    pub fn from_num_str(text: &str) -> RectangleResult<T>
    where T: std::convert::From<u32>
    {
        // Parse text into contiguous memory
        let data: Vec<Vec<T>> = text.lines()
            .enumerate()
            .map( |(row, l)|
            {
                l.chars()
                    .enumerate()
                    .map( |(col, c)| {
                        match c.to_digit(10) {
                            None => Err(RectangleError::InvalidCharacter(row, col, c)),
                            Some(n) => Ok(T::from(n))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Return
        Rectangle::from_vec(data)
    }
}

impl Rectangle<char> {
    pub fn from_char_str(text: &str) -> RectangleResult<char>{
        // Parse text into contiguous memory
        let data = text.lines()
            .map( |l|
            {
                l.chars()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Return
        Rectangle::from_vec(data)
    }
}


// ------------ Edge class ------------
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum RectangleEdge {
    Top(    (usize, usize) ),
    Bottom( (usize, usize) ),
    Left(   (usize, usize) ),
    Right(  (usize, usize) )
}

impl RectangleEdge {
    pub fn coord(&self) -> (usize, usize) {
        match self {
            RectangleEdge::Top(    c ) |
            RectangleEdge::Bottom( c ) |
            RectangleEdge::Left(   c ) |
            RectangleEdge::Right(  c ) => *c
        }
    }

    pub fn row(&self) -> usize {
        match self {
            RectangleEdge::Top(    (row, _) ) |
            RectangleEdge::Bottom( (row, _) ) |
            RectangleEdge::Left(   (row, _) ) |
            RectangleEdge::Right(  (row, _) ) => *row
        }
    }

    pub fn column(&self) -> usize {
        match self {
            RectangleEdge::Top(    (_, column) ) |
            RectangleEdge::Bottom( (_, column) ) |
            RectangleEdge::Left(   (_, column) ) |
            RectangleEdge::Right(  (_, column) ) => *column
        }
    }

    pub fn is_top(&self) -> bool {
        matches!(self, RectangleEdge::Top(_))
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, RectangleEdge::Bottom(_))
    }

    pub fn is_left(&self) -> bool {
        matches!(self, RectangleEdge::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, RectangleEdge::Right(_))
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            RectangleEdge::Top(_)  |  RectangleEdge::Bottom(_) => true,
            RectangleEdge::Left(_) |  RectangleEdge::Right(_)  => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            RectangleEdge::Top(_)  |  RectangleEdge::Bottom(_) => false,
            RectangleEdge::Left(_) |  RectangleEdge::Right(_)  => true,
        }
    }
}


// ------------ Iterator helper ------------
pub struct RectangularCoordIterator {
    width: usize,
    height: usize,
    curr_col: usize,
    curr_row: usize
}

impl RectangularCoordIterator {
    fn new(width: usize, height: usize) -> Self {
        RectangularCoordIterator {width, height, curr_col: 0, curr_row: 0}
    }
}

impl Iterator for RectangularCoordIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // Increase col
        let current = (self.curr_row, self.curr_col);
        let curr_row = self.curr_row;

        self.curr_col += 1;
        if self.curr_col >= self.width {
            self.curr_row += 1;
            self.curr_col = 0;
        }
        if curr_row >= self.height {
            return None;
        }
        Some(current)
    }
}



// ------------ Rectangular Trait ------------
pub trait Rectangular {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn iter_coord(&self) -> RectangularCoordIterator {
        // Return an iterator over the coordinates of the rectangle
        RectangularCoordIterator::new(self.get_width(), self.get_height())
    }

    fn adjacent_coordinates(&self, coord: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
        let width: usize = self.get_width();
        let height = self.get_height();

        let mut adjacent = [None; 4];

        if (1..height+1).contains(&coord.0    ) && (0..width  ).contains(&coord.1    ) { adjacent[0] = Some( (coord.0-1, coord.1  ) ); }
        if (0..height  ).contains(&(coord.0+1)) && (0..width  ).contains(&coord.1    ) { adjacent[1] = Some( (coord.0+1, coord.1  ) ); }
        if (0..height  ).contains(&coord.0    ) && (1..width+1).contains(&coord.1    ) { adjacent[2] = Some( (coord.0  , coord.1-1) ); }
        if (0..height  ).contains(&coord.0    ) && (0..width  ).contains(&(coord.1+1)) { adjacent[3] = Some( (coord.0  , coord.1+1) ); }

        adjacent
    }

    /// Returns the edges of a cell.
    ///
    /// To guarantees that unsigned-ness of the edges at cell (0, 0):
    /// * A `RectangleEdge::Vertical` edge corresponds to the left-side edge of the cell at that location
    /// * A `RectangleEdge::Horizontal` edge corresponds to the top-side edge of the cell at that location
    ///
    /// *Note*: Returned edges match the order of the cells returned by `adjacent_coordinates()`
    fn edges(&self, coord: &(usize, usize)) -> [RectangleEdge; 4] {
        //let   left_edge_coord = *coord;
        //let  right_edge_coord = (coord.0,     coord.1 + 1);
        //let    top_edge_coord = *coord;
        //let bottom_edge_coord = (coord.0 + 1, coord.1    );

        [
            RectangleEdge::Top(    *coord),
            RectangleEdge::Bottom( *coord),
            RectangleEdge::Left(   *coord),
            RectangleEdge::Right(  *coord)
        ]
    }
}

impl<T> Rectangular for Rectangle<T> {
    fn get_width(&self)  -> usize { self.width }
    fn get_height(&self) -> usize { self.height }
}


// ------------ RectangularData Trait ------------
pub trait RectangularData<T: std::cmp::PartialEq> {
    fn get(&self, coord: (usize, usize)) -> Option<&T>;

    fn adjacent_coordinates_matching(&self, coord: &(usize, usize), compare: &T) -> [Option<(usize, usize)>; 4]
    where Self: Rectangular
    {
        let mut adjacent_cells = self.adjacent_coordinates(coord);

        adjacent_cells.iter_mut()
            .for_each( |ma|
            {
                if let Some(c) = ma {
                    if self.get(*c).unwrap() != compare {
                        *ma = None;
                    }
                }
            });

        adjacent_cells
    }
}

impl<T: PartialEq> RectangularData<T> for Rectangle<T> {
    fn get(&self, coord: (usize, usize)) -> Option<&T> {
        self.data.get(coord.0).and_then(|row| row.get(coord.1) )
    }
}



// ------------ Tests ------------
#[cfg(test)]
mod tests {
    use std::vec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_build_empty_ok() {
        let input = "";
        let square: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

        assert_eq!(square.data, Vec::<Vec<u32>>::new());
        assert_eq!(square.width,  0);
        assert_eq!(square.height, 0);
    }


    #[test]
    fn test_build_square_u32_ok() {
        let input = "123
456
789";
        let square: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

        assert_eq!(square.data, vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]);
        assert_eq!(square.width,  3);
        assert_eq!(square.height, 3);
    }

    #[test]
    fn test_build_square_char_ok() {
        let input = "abc
def
ghi";
        let square: Rectangle<char> = Rectangle::from_char_str(input).unwrap();
        assert_eq!(square.data, vec![vec!['a','b','c'],vec!['d','e','f'],vec!['g','h','i']]);
        assert_eq!(square.width,  3);
        assert_eq!(square.height, 3);
    }

    #[test]
    fn test_build_rectangle_u32_ok() {
        let input = "123
456
789
000
555";
        let rectangle: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

        assert_eq!(rectangle.data, vec![vec![1,2,3],vec![4,5,6],vec![7,8,9],vec![0,0,0],vec![5,5,5]]);
        assert_eq!(rectangle.width,  3);
        assert_eq!(rectangle.height, 5);
    }

    #[test]
    fn test_build_rectangle_u32_different_widths() {
        let input = "123
456
789
000
5555";
        let rectangle_result: RectangleResult<u32> = Rectangle::from_num_str(input);

        assert_eq!(rectangle_result.unwrap_err(), RectangleError::DifferentWidths);
    }

    #[test]
    fn test_build_rectangle_u32_bad_parsing() {
        let input = "123
45a
789
000";
        let rectangle_result: RectangleResult<u32> = Rectangle::from_num_str(input);

        assert_eq!(rectangle_result.unwrap_err(), RectangleError::InvalidCharacter(1, 2, 'a'));
    }

    #[test]
    fn test_adjacent_coordinates() {
        let input = "123
456
789";
        let rectangle: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

        assert_eq!(
            rectangle.adjacent_coordinates(&(1, 1)),
            [Some( (0, 1) ), Some( (2, 1) ), Some( (1, 0) ), Some( (1, 2) )]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(0, 0)),
            [ None, Some( (1, 0) ), None, Some( (0, 1) ) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(0, 1)),
            [ None, Some( (1, 1) ), Some( (0, 0) ), Some( (0, 2) ) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(1, 0)),
            [ Some( (0, 0) ), Some( (2, 0) ), None, Some( (1, 1) ) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(2, 2)),
            [ Some( (1, 2) ), None, Some( (2, 1) ), None ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(2, 1)),
            [ Some( (1, 1) ), None, Some( (2, 0) ), Some( (2, 2) ) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates(&(1, 2)),
            [ Some( (0, 2) ), Some( (2, 2) ), Some( (1, 1) ), None ]
        );
    }


    #[test]
    fn test_iter_coord() {
        let input = "12
34
56";
    let rectangle: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

    assert_eq!(
        rectangle.iter_coord().collect::<Vec<_>>(),
        vec!{ (0, 0), (0, 1),
              (1, 0), (1, 1),
              (2, 0), (2, 1) }
        );
    }

    #[test]
    fn test_adjacent_coordinates_matching() {
        let input = "121
416
181";
        let rectangle: Rectangle<u32> = Rectangle::from_num_str(input).unwrap();

        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(1, 1), &1),
            [None, None, None, None]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(0, 0), &1),
            [None, None, None, None]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(0, 1), &1),
            [ None, Some( (1, 1) ), Some( (0, 0)), Some( (0, 2)) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(1, 0), &1),
            [ Some( (0, 0) ), Some( (2, 0)), None, Some( (1, 1)) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(2, 2), &1),
            [None, None, None, None]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(2, 1), &1),
            [ Some( (1, 1) ), None, Some( (2, 0)), Some( (2, 2)) ]
        );
        assert_eq!(
            rectangle.adjacent_coordinates_matching(&(1, 2), &1),
            [ Some( (0, 2) ), Some( (2, 2)), Some( (1, 1)), None ]
        );
    }
}
