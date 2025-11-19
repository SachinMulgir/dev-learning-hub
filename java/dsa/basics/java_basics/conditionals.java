package java.dsa.basics.java_basics;

import java.util.Scanner;

public class conditionals {
    public static void main(String[] args) {
        // conditionals: if/else && switch:

        int a = 10;
        
        // ---------------------------- if-else case ------------------------
        // using if else:
        if (a < 5) {
            System.out.println("variable a:" + a + " is less than 5.");
        } else {
            System.out.println("variable a:" + a + " is greater than 5.");
        }

        System.out.println();
        System.out.println();



        // ---------------------------- multiple if else case ------------------------
        // using nested if-else:
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter your marks to get the respective Grade : ");
        double marks = sc.nextDouble();                     // taking user input for the marks.

        // now conditional:
        // if marks:
        // 100 >= marks > 90  -> Grade A
        // 90 >= marks > 80   -> Grade B
        // 80 >= marks > 70   -> Grade C
        // 70 >= marks > 60   -> Grade D
        // 30 >= marks > 40   -> Grade E
        // else               -> Grade F

        if (marks > 90 && marks <= 100) {
            System.out.println("Marks: " + marks + " -> Grade A+");
        } else if (marks > 80 && marks <= 90) {
            System.out.println("Marks: " + marks + " -> Grade A");
        } else if (marks > 70 && marks <= 80) {
            System.out.println("Marks: " + marks + " -> Grade B");
        } else if (marks > 60 && marks <= 70) {
            System.out.println("Marks: " + marks + " -> Grade C");
        } else if (marks > 40 && marks <= 60) {
            System.out.println("Marks: " + marks + " -> Grade D");
        } else if (marks > 30 && marks <= 40) {
            System.out.println("Marks: " + marks + " -> Grade E");
        } else {
            System.out.println("Marks: " + marks + " -> Grade F");
        }


        System.out.println();
        System.out.println();

        // ---------------------------- switch case ------------------------
        // we'll use the below example for the same:
        // enter a number to get the below funtionality:
        // 1. add
        // 2. substract
        // 3. multiply
        // 4. divide

        System.out.println("1. Add");
        System.out.println("2. Substract");
        System.out.println("3. Multiply");
        System.out.println("4. Divide");
        System.out.println("Enter your choice:");
        int choice = sc.nextInt();

        System.out.println("enter 1st number:");
        int num1 = sc.nextInt();
        System.out.println("enter 2nd number:");
        int num2 = sc.nextInt();

        System.out.println();

        switch (choice) {
            case 1 : 
            System.out.println("Result -> " + num1 + " + " + num2 + " = " + (num1+num2));
            break;
            case 2 : 
            System.out.println("Result -> " + num1 + " - " + num2 + " = " + (num1-num2));
            break;
            case 3 : 
            System.out.println("Result -> " + num1 + " * " + num2 + " = " + (num1*num2));
            break;
            case 4 : 
            System.out.println("Result -> " + num1 + " / " + num2 + " = " + (num1/num2));
            break;
            default:
            System.out.println("Invalid Input from the user.");
        };

        sc.close();
    }
}
