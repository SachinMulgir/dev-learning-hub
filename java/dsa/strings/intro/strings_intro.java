package java.dsa.strings.intro;
import java.util.Scanner;

public class strings_intro {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        // Strings again are non-primitive datatype:
        // Derivation:
        String s = "Sachin";

        // User input string: 
        System.out.println("Enter word:");
        String word = sc.next();                // allows to take a input for single word
        System.out.println("Entered word: " + word);
        
        System.out.println("Enter line:");
        String line = sc.nextLine();            // allows to take input for a complete line
        System.out.println("Entered line: " + line);

        // Functions in string:
        string_functions();


        // NOTE:
        // Strings are immutable, therefore we cannot change the defined string. We have to assign the new value to a new variable.
        // 
        //
        //
        //
        //
        //
        //

        sc.close();
    }

    public static void string_functions() {
        // 1. Concatenation : We can concatenate two strings using '+'
        String hello = "hello";
        String world = "world";
        String hello_world = hello + world;

        System.out.println("hello : '" + hello + "'' && world : '" + world + "' -> concat : '" + hello_world);


        // 2. Length : String.length();
        String s1 = "Hi, this is a sample string to check the length of it.";
        int len_s1 = s1.length();



    }
}
