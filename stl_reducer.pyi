from pathlib import Path
from typing import List, Tuple

def reduce_stl_points(
    stl_file_path: Path, clusters: int
) -> List[Tuple[float, float, float]]:
    """Reads an STL file and reduces the number of points.

    Args:
        stl_file_path (Path): Path to the STL file.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        List[List[float]]: List of points in the reduced STL file.
    """
