# Rust/WebAssembly Notebooks

# Installing 

We will be assuming the users of these notebooks are using either Linux or Windows Subsystem for Linux (WSL). Users of other Operating Systems may still use the notebooks if they can get the required applications installed.

To begin using rust, users will need the following

    - Anaconda/Miniconda
    - Jupyter Notebooks
    - Rustup
    - EvCxR Jupyter Kernel


## Setting up the Environment

### Anaconda/Miniconda and Jupyter Notebooks

If you have Anaconda or Miniconda installed on your system you may skip this.

Install Miniconda by following the instrutions here [https://docs.conda.io/en/latest/miniconda.html](https://docs.conda.io/en/latest/miniconda.html). You can choose to install Anaconda if you prefer, but it is a larger installation.



```bash 
    conda install -c conda-forge notebook    
```

### Rustup

If your using WSL you can install rustup quickly using the following console command, otherwise go to the website and download an installer.

```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have rustup installed, check if it installed correctly and that you have the most current version

```bash
    rustup check
    rustup update
```

Then install the Rust Kernal for Jupyter Notebooks, EvCxR, found here: [https://github.com/google/evcxr/tree/master/evcxr_jupyter](https://github.com/google/evcxr/tree/master/evcxr_jupyter)

```bash
    cargo install evcxr_jupyter
    evcxr_jupyter --install
```


##  Running the notebook

Once you have everything installed properly, a notebook server can be launched from any directory. To run this notebook run the following code in the WSL or Bash shell

```bash
    cd <clone directory>
    jupyter notebook
```

## Contributing

Send a Pull Request if anyone wishes to contribute

## Copyright/Licensing

&copy; James E. Halladay.

Protected under the terms of the MIT License. Permission to distribute, modify, and copy is granted. License file included in notebooks main directory for reference.
