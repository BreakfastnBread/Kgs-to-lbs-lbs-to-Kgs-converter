weight = float(input("Input your weight: "))
unit = input("(K)gs or (L)bs: ")
match unit:
    case "K":
        solved = weight / 0.45
        print(solved)
    case "L":
        solved2 = weight * 0.45
        print(solved2)