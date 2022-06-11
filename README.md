
sound-log
-----------

Log events and system load with sound

### install

```bash
# assuming ubuntu or debian
sudo apt install libasound2-dev
# clone and move to repo directory
cargo build
```

### run

```bash
target/debug/sound-log --cpu
```

### credit

* conversations with Zahary Karadjov and others about debugging/logging
* peep: http://peep.sourceforge.net/intro.html , https://www.usenix.org/legacy/publications/library/proceedings/lisa2000/full_papers/gilfix/gilfix_html/index.html
* other software/tooling/commments/discussion that I am not sure about right now
* my job: related to debugging and logging and many debugging tools/concepts

existing software/tooling:

from https://news.ycombinator.com/item?id=24850583

* using: `systemstat` (system state/load) and `rodio` libraries (audio playback library)

### license

MIT

