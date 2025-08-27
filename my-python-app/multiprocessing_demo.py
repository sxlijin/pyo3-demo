#!/usr/bin/env python3
"""
Minimal example demonstrating pickling errors with PyO3 objects and multiprocessing.Pool
Based on: https://laszukdawid.com/blog/2017/12/13/multiprocessing-in-python-all-about-pickling/
"""

import multiprocessing
import os
import threading
import time
import my_rust_crate

o = my_rust_crate.MyRustObject("lorem ipsum")


def process_rust_object(i: int):
    """Function that tries to process a PyO3 object."""
    pid = os.getpid()
    tid = threading.get_ident()
    time.sleep(1)
    print(f"processing with rust object {i} - PID: {pid}, Thread ID: {tid}")
    o.print()
    print(o)
    return "Processed"


def main():
    print("\nAttempting to use multiprocessing.Pool with PyO3 objects...")
    print("This will fail with a pickling error:\n")
    
    with multiprocessing.Pool(processes=2) as pool:
        # This will fail because MyRustObject cannot be pickled
        results = pool.map(process_rust_object, [1, 2, 3, 4, 5], chunksize=1)
        print(f"Results: {results}")


if __name__ == "__main__":
    main()
