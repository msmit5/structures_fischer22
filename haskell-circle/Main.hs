import Data.Either

main = do
  putStrLn "Please input the diameter of a circle"
  diaUIO <- readLn :: IO Double
  let dia = diaUIO
  putStrLn $ "You Input: " ++ (show dia)

  let res = either lCase rCase (circleHandler dia)
  putStrLn res


lCase :: [Char] -> [Char]
lCase l  = "ERROR:\n" ++ l

rCase :: [Double] -> [Char]
rCase r = do
  printStrResults $ zip ["Radius: ", "Circumfrence: ", "Area: "] r

printStrResults x = printStrResults' x ""

printStrResults' dat cur
  | dat == [] = cur
  | otherwise = do
    let x = head dat
    printStrResults' (tail dat) (cur ++ ((fst x) ++ (show $ snd x)) ++ "\n")

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
