# quickarrand

This is an algorithm to randomize a vector of bools, with a variable chance out of 100 to randomize each element, which can be faster than the naive method of simply looping through each element.

It works by jumping forward by a random amount that keeps the ratio of trues/falses the same as the standard method. This is especially fast for situations with large arrays, and low chance for each element to be set on.

![image](https://user-images.githubusercontent.com/104919666/176510318-51980e0c-065b-47d4-b006-2a9c41fc68a4.png)
