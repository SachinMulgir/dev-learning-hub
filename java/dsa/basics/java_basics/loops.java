package java.dsa.basics.java_basics;

public class loops {
    public static void main(String[] args) {
        // loops
        // we have 3 types of loops in java:
        // 1. for loop : runs for the mentioned number of iterations.
        // 2. while loop : runs till the condition provided is true.
        // 3. do while loop : runs like while loop, but 1st iteration is done by default.

        //1. for loop:
        for (int i = 0; i < 5; i++) {
            System.out.println("i : " + i);
        }

        // here:
        // int i = 0 -> means the loop with start with a variable 'i' of 'int' type with value '0'.
        // i < 5     -> it will run till the value of 'i' is less than 5.
        // i++       -> On each iteration the value of 'i' will increase by 1.

        // This loop prints the value of i from 0 to 4 : 0 1 2 3 4



        
    }
}
