def process_input(filename: str) -> list[list[int]]:
    reports = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            report = []
            nums = line.split(' ')
            for num in nums:
                report.append(int(num))
            reports.append(report)
    return reports

def check_report(report: list[int]) -> bool:
    if len(report) == 1:
        return True
    if report[1] == report[0]:
        return False
    elif report[1] > report[0]:
        ascending = True
    else:
        ascending = False
    for i in range(1, len(report)):
        if ascending:
            if report[i] <= report[i-1]:
                return False
            if report[i] - report[i-1] > 3:
                return False
        else:
            if report[i-1] <= report[i]:
                return False
            if report[i-1] - report[i] > 3:
                return False
    return True

def count_reports(reports: list[list[int]]) -> int:
    safe_reports = 0
    for report in reports:
        if check_report(report):
            safe_reports +=1
    return safe_reports

if __name__ == "__main__":
    #reports = [[7,6,4,2,1],[1,2,7,8,9],[9,7,6,2,1],[1,3,2,4,5],[8,6,4,4,1],[1,3,6,7,9]]
    reports = process_input('input-02.txt')

    safe_reports = count_reports(reports)
    print(f"Number of safe reports is {safe_reports}")
