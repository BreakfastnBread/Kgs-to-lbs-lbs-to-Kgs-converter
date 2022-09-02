weight = float(input("Input your weight: "))
unit = input("(K)g or (L)bs: ").upper()
match unit:
    case "K":
        converted = weight / 0.45
        print("Weight in Lbs: " + str(converted))
    case "L":
        converted = weight * 0.45
        print("Weight in Kgs: " + str(converted))
    case _:
        print("unrecognised input: " + unit)
