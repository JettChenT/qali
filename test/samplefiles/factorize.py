n = int(input("Enter the number to factorize:"))

outlst = []
for i in range(2, n+1):
    k = 0
    while(n % i == 0):
        k+=1
        n = n // i
    if k > 0:
        if k>1:
            outlst.append(f"{i}^{k}")
        else:
            outlst.append(f"{i}")

print("*".join(outlst))