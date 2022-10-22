vals = "[(0,3),(1,7),(2,5),(4,8),(6,9),(10,13),(11,15),(12,18),(14,17),(16,19)],[(0,14),(1,11),(2,16),(3,17),(4,12),(5,19),(6,10),(7,15),(8,18),(9,13)],[(0,4),(1,2),(3,8),(5,7),(11,16),(12,14),(15,19),(17,18)],[(1,6),(2,12),(3,5),(4,11),(7,17),(8,15),(13,18),(14,16)],[(0,1),(2,6),(7,10),(9,12),(13,17),(18,19)],[(1,6),(5,9),(7,11),(8,12),(10,14),(13,18)],[(3,5),(4,7),(8,10),(9,11),(12,15),(14,16)],[(1,3),(2,4),(5,7),(6,10),(9,13),(12,14),(15,17),(16,18)],[(1,2),(3,4),(6,7),(8,9),(10,11),(12,13),(15,16),(17,18)],[(2,3),(4,6),(5,8),(7,9),(10,12),(11,14),(13,15),(16,17)],[(4,5),(6,8),(7,10),(9,12),(11,13),(14,15)],[(3,4),(5,6),(7,8),(9,10),(11,12),(13,14),(15,16),]"

for pair in vals.replace('[', '').replace(']', '').split("),"):
    parts = pair[1:].split(",")
    if len(parts) != 2:
        continue

    a = int(parts[0])
    b = int(parts[1])

    print(f"swap_if_less(arr_ptr, {a}, {b}, is_less);")
