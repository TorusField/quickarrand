# quickarrand

This is an algorithm to randomize a vector of bools, with a variable chance out of 100 to randomize each element, which can be faster than the naive method of simply looping through each element.

It works by jumping forward by a random amount that keeps the ratio of trues/falses the same as the standard method. This is especially fast for situations with large arrays, and low chance for each element to be set on.

![image](https://user-images.githubusercontent.com/104919666/176509246-75b72e6e-801b-4b2f-a7b7-678971bc0a41.png)
