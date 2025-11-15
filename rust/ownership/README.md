# 🦀 Rust Ownership & Borrowing

Rust ensures memory safety **without a garbage collector** using Ownership and Borrowing rules.  
This document summarizes all key concepts in a clean, interview-ready format.

---

## 🔑 Ownership

Ownership is the foundation of Rust’s memory model.

### **Rules of Ownership**
1. **Each value has exactly one owner.**
2. **Only one owner exists at any given time.**
3. **When an owner goes out of scope, the value is automatically dropped.**

---

## ❓ Why Two Owners Cannot Co-Exist

### **1. Both owners go out of scope**
- Both will attempt to free the same memory.
- This causes a **double free error**, leading to:
  - Memory corruption  
  - Security vulnerabilities  
  - Undefined behavior  

### **2. One owner goes out of scope**
- That owner frees the memory.
- The other owner now points to freed memory → **dangling pointer**.
- Future operations crash or behave unpredictably.

---

## 🔄 Borrowing

Borrowing means *referring to data without taking ownership*.

### **Types of Borrows**

#### ✔️ Immutable Reference  
`&T`  
- Read-only access  
- Multiple allowed simultaneously  

#### ✔️ Mutable Reference  
`&mut T`  
- Exclusive write access  
- Only one allowed at a time  

---

## 📏 Borrowing Rules

| Borrow Combination | Allowed? | Explanation |
|--------------------|----------|-------------|
| `&T` + `&T`        | ✅ Yes   | Many readers allowed |
| One `&mut T`       | ✅ Yes   | Only one writer |
| `&mut T` + `&mut T`| ❌ No    | Multiple writers cause conflicts |
| `&T` + `&mut T`    | ❌ No    | Can't mix readers & writer |

> ⚠️ **Golden Rule:**  
> **Many Readers OR One Writer — Never Both**

---

## ❓ Why Two Mutable References Are Forbidden?

Because they can cause a **Data Race**, which Rust prevents at compile-time.

## ⚡ Data Race Condition

A **data race** occurs when:

1. **Concurrent Access**  
   Multiple references access the same data at the same time.

2. **Shared Memory**  
   They point to the same memory location.

3. **At Least One Writer**  
   One access modifies the data.

4. **No Synchronization**  
   No locking or coordination mechanism is used.

Rust ensures such scenarios never compile, guaranteeing memory safety.

---

## ✅ Summary

- Rust uses ownership & borrowing to ensure memory safety.
- Ownership rules prevent double-free and dangling pointers.
- Borrow rules prevent data races.
- The compiler enforces safety at compile time, not runtime.
