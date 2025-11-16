package DSA.java_basics;

import java.util.Scanner;

public class input_output {
    public static void main(String[] args) {

        System.out.println("Hello World");

        Scanner sc = new Scanner(System.in);

        System.out.println("Integer:");
        int a = sc.nextInt();
        
        System.out.println("Float:");
        float b = sc.nextFloat();
        
        System.out.println("Double:");
        double d = sc.nextDouble();
        
        System.out.println("Short:");
        short sh = sc.nextShort();
        
        System.out.println("Long:");
        long l = sc.nextLong();
        
        System.out.println("Byte:");
        byte by = sc.nextByte();
        
        System.out.println("Boolean:");
        boolean bool = sc.nextBoolean();
        
        System.out.println("Word (single string):");
        String word = sc.next();

        System.out.println("Character:");
        char ch2 = sc.next().charAt(0);

        sc.nextLine(); // ✅ clear buffer

        System.out.println("Full String:");
        String s = sc.nextLine();

        sc.close();                                                                       // Here we close the scanner.

        // ✅ Printing all inputs
        System.out.println("\n------ OUTPUT VALUES ------");
        System.out.println("Integer  : " + a);
        System.out.println("Float    : " + b);
        System.out.println("Double   : " + d);
        System.out.println("Short    : " + sh);
        System.out.println("Long     : " + l);
        System.out.println("Byte     : " + by);
        System.out.println("Boolean  : " + bool);
        System.out.println("Word     : " + word);
        System.out.println("Char     : " + ch2);
        System.out.println("String   : " + s);
    }
}
