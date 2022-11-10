my_gcd :: Integer -> Integer -> Integer 
my_gcd n1 n2 = my_gcd' n1 n2 1 0

my_gcd' :: Integer -> Integer -> Integer -> Integer -> Integer
my_gcd' n1 n2 i res 
  | not $ (i <= n1) || (i <= n2) = res -- stop recursion if i is not less than or equal to n1 or n2
  | (&&) (0 == mod n1 i) (0 == mod n2 i) = my_gcd' n1 n2 (i + 1) i
  | otherwise = my_gcd' n1 n2 (i + 1) res 
