# Two Sum

**Difficulty:** Easy  
**URL:** https://leetcode/problems/two-sum  

## Description

Given an array of integers `nums`&nbsp;and an integer `target`, return *indices of the two numbers such that they add up to `target`*.



You may assume that each input would have ***exactly* one solution**, and you may not use the *same* element twice.



You can return the answer in any order.



&nbsp;


<strong class="example">Example 1:**



```

**Input:** nums = [2,7,11,15], target = 9
**Output:** [0,1]
**Explanation:** Because nums[0] + nums[1] == 9, we return [0, 1].

```

<strong class="example">Example 2:**



```

**Input:** nums = [3,2,4], target = 6
**Output:** [1,2]

```

<strong class="example">Example 3:**



```

**Input:** nums = [3,3], target = 6
**Output:** [0,1]

```

&nbsp;


**Constraints:**




	- `2 <= nums.length <= 10<sup>4</sup>`
	- `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
	- `-10<sup>9</sup> <= target <= 10<sup>9</sup>`
	- **Only one valid answer exists.**


&nbsp;


**Follow-up:&nbsp;**Can you come up with an algorithm that is less than `O(n<sup>2</sup>)`<font face="monospace">&nbsp;</font>time complexity?

## Examples

### Example 1

```
[2,7,11,15]
```

### Example 2

```
9
```

### Example 3

```
[3,2,4]
```

### Example 4

```
6
```

### Example 5

```
[3,3]
```

### Example 6

```
6
```

## Constraints

* TODO: Add constraints from problem description

## Topics

- Array
- Hash Table

## Hints

1. A really brute force way would be to search for all possible pairs of numbers but that would be too slow. Again, it's best to try out brute force solutions just for completeness. It is from these brute force solutions that you can come up with optimizations.

2. So, if we fix one of the numbers, say <code>x</code>, we have to scan the entire array to find the next number <code>y</code> which is <code>value - x</code> where value is the input parameter. Can we change our array somehow so that this search becomes faster?

3. The second train of thought is, without changing the array, can we use additional space somehow? Like maybe a hash map to speed up the search?

## Solution Approach

<!-- Write your approach here -->

### Complexity Analysis

- **Time Complexity:** O()
- **Space Complexity:** O()
