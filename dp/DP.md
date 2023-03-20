# Dynamic programming

## Strategy for Dynamic programming problems 

1. State the sub problem in words. This usually involves stating the same problem statement on a prefix of the input. For example in case of the Longest increasing subsequence problem the sub problem in words will be : "Let L(i) = Length of the longest increasing subsequence for the first i elements from the input list."

2. Find the recurrence relation. This involves expressing the solution to the ith sub-problem in terms of smaller sub-problems. So we will have to express L(i) in terms of L(1) .... L(i - 1)

A key point to note is that we have to re-formulate the sub-problem definitions in most cases.

