# binary_search_rust
Binary search, also known as half-interval search, logarithmic search, or binary chop, is a search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array.
However, binary search can be used to solve a wider range of problems, such as finding the next-smallest or next-largest element in the array relative to the target even if it is absent from the array. <br>
Worst-case performance	O(log n)
Best-case performance	O(1)
Average performance	O(log n)
Worst-case space complexity O(1) <br>

Binary search begin by comparing the an element in the middle of the array with the target value. If the target value matches the element, its position in the array is returned. If the target value is less than element , the search continues in the lower half of the array. If the target is greater than the element, the search continues in the upper half of the array. By doing this the algorithm eliminate the half in which the target value cannot lie in each iteration. <br>
https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Binary-search-work.gif/440px-Binary-search-work.gif
