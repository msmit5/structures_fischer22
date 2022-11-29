import Text.Read
import Text.Printf

main = do
  putStrLn "Please input the diameter of a circle"
  putStr "> "
  diaAsStr <- getLine

  case readMaybe diaAsStr :: Maybe Double of 
    Just x -> do
      putStrLn $ "You input: " ++ (show x) ++ "\n"
      putStrLn $ either lCase rCase (circleHandler x)
    Nothing -> putStrLn "Invalid Input"


lCase :: [Char] -> [Char]
lCase l  = "ERROR:\n" ++ l

rCase :: [Double] -> [Char]
rCase r = do
  printStrResults $ zip ["Radius:       ", "Circumfrence: ", "Area:         "] r

printStrResults x = printStrResults' x ""

printStrResults' dat cur
  | dat == [] = cur
  | otherwise = do
    let x = head dat
    --printStrResults' (tail dat) (cur ++ ((fst x) ++ (show $ snd x)) ++ "\n")
    printStrResults' (tail dat) (cur ++ ((fst x) ++ (trunc (snd x)) ++ "\n"))

trunc x = printf "%.4f" x


circleHandler :: Double -> Either [Char] [Double]
circleHandler dia
  | dia <= 0 = Left "Data cannot be less than or equal to 0"
  | otherwise = Right (map ($ dia) [getRad, getCircum, getArea])


getRad :: Double -> Double
getRad x = 0.5 * x

getCircum :: Double -> Double
getCircum x = pi * x

getArea :: Double -> Double
getArea x = (pi) * (square $ getRad x)

square :: Double -> Double
square x = x * x
