from pathlib import Path
from typing import List, Tuple

def reduce_stl_points(
    stl_file_path: Path, clusters: int
) -> Tuple[List[Tuple[float, float, float]], List[Tuple[float, float, float]]]:
    """Reduces the number of points in an STL file using fast grid clustering.

    Args:
        stl_file_path (Path): Path to the STL file.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        Tuple[List[Tuple[float, float, float]], List[Tuple[float, float, float]]]: Two lists of tuples containing the reduced points and normals.
    """
