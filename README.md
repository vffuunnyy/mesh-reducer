# STL Reducer

This project provides a Rust implementation for reading STL files and reducing the number of points using a fast grid sampling method. The functionality is exposed to Python using PyO3.

## Features

- Read STL files and extract points.
- Reduce the number of points using a fast grid sampling method.
- Python bindings for easy integration with Python projects.

## Usage

### Python

```bash
pip install stl-reducer
```

## Functions

#### `reduce_stl_points(stl_file_path: Path, clusters: int) -> List[Tuple[float, float, float]]`

Reads an STL file and reduces the number of points.

**Args:**
- `stl_file_path (Path)`: Path to the STL file.
- `clusters (int)`: Number of clusters to reduce the points to.

**Returns:**
- `List[Tuple[float, float, float]]`: List of points in the reduced STL file.

## License

This project is licensed under the MIT License.