import csv

NUMBER_OF_BYTES = 16
NUMBER_OF_PLAINTEXTS = 150
NUMBER_OF_TRACES = 50000
LIMIT = 0.8


# Traces 150x{X}
def save_traces(byte_number, traces):
    # Write the traces in a file
    print("Saving traces in trace"+str(byte_number)+".txt")
    print("Size of traces: "+str(len(traces)))
    print("Size of each trace: "+str(len(traces[0])))

    with open(f'./data/trace{byte_number}.txt', 'w') as f:
        for plaintext in traces:
            # Save list in separated by space
            f.write('\t'.join(map(str, plaintext)))
            f.write('\t\n')


def analysis_dataset2():
    for byte in range(NUMBER_OF_BYTES):
        trace_byte = []
        clock_byte = []
        print("Analyzing byte "+str(byte))

        # Read dataset{byte}.txt
        with open('./dataset2/trace'+str(byte)+'.txt') as csvfile:
            reader = csv.reader(csvfile, delimiter='\t')
            for element in reader:
                trace_byte.append(element)
        trace_byte = [[float(num) for num in sublist]
                      for sublist in trace_byte]

        trace_flank = []

        # Read clock{byte}.txt
        with open('./dataset2/clock'+str(byte)+'.txt') as csvfile:
            reader = csv.reader(csvfile, delimiter='\t')
            for element in reader:
                clock_byte.append(element)
        clock_byte = [[float(num) for num in sublist]
                      for sublist in clock_byte]

        for i in range(NUMBER_OF_PLAINTEXTS):
            aux = []
            for j in range(NUMBER_OF_TRACES):
                if (j > 5 and j < 49995 and clock_byte[i][j-1] < LIMIT and clock_byte[i][j] >= LIMIT):
                    for x in range(10):  # We store the 10 traces for each rising flank
                        aux.append(trace_byte[i][j-5+x])
            trace_flank.append(aux)

        # Save aligned traces in a file
        save_traces(byte, trace_flank)


if __name__ == "__main__":
    analysis_dataset2()
