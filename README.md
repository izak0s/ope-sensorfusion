# Fault-tolerant and privacy preserving sensor fusion system based on order-preserving encryption
> **Disclaimer**: This project is part of my academic thesis and serves primarily as an experimental proof-of-concept implementation. It is not optimized for production use and may not deliver the highest performance. Additionally, as I learned Rust during my thesis, the code might not adhere to all best practices. However, I'm open to feedback and contributions to improve the project code!

This project proposes a design that integrates order-preserving encryption (OPE) into sensor fusion systems to enhance privacy while maintaining fault tolerance.  By encrypting individual sensor values while preserving their order, the system ensures privacy without compromising reliability, even in the presence of potentially malicious sensors. The fault-tolerant algorithm, Marzullo's algorithm, is performed directly on encrypted data to determine optimal sensor while preserving privacy. 


## Installation
Ensure Rust is installed on your system. The installation instructions can be found on the official [Rust website](https://www.rust-lang.org/tools/install). This code has been tested exclusively on Linux and MacOS platforms. 

## Demo
![demo.png](demo.png)

## Usage

The `scripts` directory contains the following scripts: `build.sh`, `run_collector.sh`, `run_client.sh`, and `run_sensor.sh`.

### Building the Program

Before running any scripts, ensure to compile the program(s) by executing `build.sh`.

```bash
./build.sh
```

If you encounter a permission error while executing the scripts, you may need to make them executable. Run the following command in the `scripts` directory:

```bash
chmod +x *.sh
```


### Running Sensors

The `run_sensor.sh` script is used to start sensors. It takes two arguments:

```bash
./run_sensor.sh <sensor_id> <faulty>
```

- `<sensor_id>`: The ID of the sensor, starting from 1. The listening port of the sensor increases from 13300 + the sensor ID.
- `<faulty>`: If set to `true`, the sensor will generate random values as encrypted sensor values instead of actual values, acting as malicious sensor.

Example (2 honest sensors, 1 malicious):

```bash
./run_sensor.sh 1 false
./run_sensor.sh 2 false
./run_sensor.sh 3 true
```

### Running the Collector

The `run_collector.sh` script is used to start the collector. It takes one argument:

```bash
./run_collector.sh <number_of_sensors>
```

- `<number_of_sensors>`: The number of sensors the collector will retrieve data from.

Example:

```bash
./run_collector.sh 3
```
### Running the client

The `run_client.sh` script is used to start the client and make a request to the sensor collector. It does not take any arguments:

```bash
./run_collector.sh
```

## License
See [LICENSE](LICENSE)

---
