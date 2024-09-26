# RC: Rapid Copy

RC (Rapid Copy) is an ultra-fast, multi-process file copying tool. Designed to offer exceptional performance, RC leverages modern hardware capabilities to maximize copying speed.

## Key Features

- **Ultra-fast copying**: Uses multi-processing and asynchronous I/O techniques to achieve superior copying speeds.
- **Intelligent hardware detection**: Automatically optimizes performance based on storage type (SSD, HDD, etc.).
- **Multiple operation modes**: Includes file modes, attribute preservation, recursive copying, and more (in short, generally everything that Unix cp has).
- **Robust command-line interface**: Offers a wide range of options to customize copying behavior.
- **Efficient error handling**: Provides detailed information and debugging options for troubleshooting.

## Advanced Techniques Used

1. **Adaptive multi-processing**: RC dynamically adjusts the number of threads based on available hardware and storage type.
2. **Optimized buffering**: Uses an 8 MB buffer size to maximize I/O performance.
3. **Asynchronous I/O handling**: Leverages Rust's capabilities for non-blocking I/O, improving performance in disk operations.

## Installation

To install the project, simply run this command if you have cargo:

```bash
cargo install --git https://github.com/GoldenHand-Dev/rc
```
or if you want to install it another way:
```bash
cargo install rapidcopy
```

## Basic Usage

```bash
rc [OPTIONS] SOURCE DESTINATION
```

For a complete list of options, run:

```bash
rc --help
```

## Usage Examples

1. Copy a file:
   ```
   rc sales_report.xlsx /home/user/Documents/sales_report_copy.xlsx
   ```

2. Copy a directory recursively:
   ```
   rc -r /home/user/Photos /media/backup/Photos_2024
   ```

3. Copy preserving attributes:
   ```
   rc -p signed_contract.pdf /home/user/Legal_Documents/signed_contract.pdf
   ```

4. Copy in verbose mode:
   ```
   rc -v presentation.pptx /home/user/Work/final_presentation.pptx
   ```

5. Copy without overwriting existing files:
   ```
   rc -n customer_data.csv /home/user/CRM/new_customer_data.csv
   ```

6. Copy with a specific number of threads:
   ```
   rc --threads 4 4k_movie.mp4 /media/usb/4k_movie_copy.mp4
   ```

7. Copy in interactive mode:
   ```
   rc -i /home/user/Downloads/* /media/external/Downloads_Backup/
   ```

8. Copy and update only if the source is newer:
   ```
   rc -u database.sql /home/user/Backups/updated_database.sql
   ```

For a complete list of options, refer to the command help (`rc --help`).

## License

RC is licensed under the GNU General Public License v3.0. See the LICENSE file for more details.

## Contact

To report issues or suggest improvements, please open an issue in our repository or contact me at 1nu55et1@gmail.com (please, if you use Gmail, put something like "Rapid Fast issue" or "Rapid Fast recommendation" in the subject so it will be easier for me to know what they are).

---

RC: Making file copying fast, efficient, and hassle-free.

---

Notice: This project is primarily for personal use, so don't expect much attention to it. But what happened and the reason for its creation is that cp doesn't exist on Windows and PowerShell doesn't work well on my 2-thread, 8GB RAM computer. So what I did is create this program for daily use. Don't expect it to be the best but rather something good and simple above all, but since I usually use cp a lot, I ended up including all the Unix cp options and maybe more in the future.
