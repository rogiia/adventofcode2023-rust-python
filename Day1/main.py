num_map = {  
'one': '1',  
'two': '2',  
'three': '3',  
'four': '4',  
'five' : '5',  
'six': '6',  
'seven': '7',  
'eight': '8',  
'nine': '9'  
}  
nums = num_map.keys()  
max_len = 5  
to_sumup = []  
with open('input.txt') as f:  
    for line in f:  
        found_nums = []  
        line = line.strip()  
        for idx, char in enumerate(line):   
            try:  
                found = int(char)  
                found_nums.append(char)  
                continue  
            except ValueError:   
                for i in range(1, max_len+1):  
                    if line[idx:idx+i] in nums:  
                        found_nums.append(num_map[line[idx:idx+i]])  
                        break  
        to_sumup.append(int(found_nums[0]+found_nums[-1]))
        print(f"{found_nums[0]}, {found_nums[-1]}, {line}")


print(f"Result: sum(to_sumup)")