# Todo
## Match against window title (before, and) in addition to class name
## Add click events to each icon / workspace
## Add pygmenter 
1. The pygmenter should have different modes for debugging. Terminal and Polybar pygmenting
2. Find a different name than pygmenter. Annotators?
3. Configurable annotators that inject strings with colors and events 
4. Accent annotator adds the accent based on settings and node state 
5. Color annotator adds color based on settings and node state 
6. Action annotator adds click events
7. Default annotator settings 
## Fix output format including subscripts / superscripts
## Support only showing the current monitor somehow
## Create a common interface which can support bspwm state as well as other DEs (and my NATS thing)
## Make socket path configurable somehow 
1. We must support multiple concurrent instances, e.g. different instances for different bars. (or multiple per bar, even)
2. We must support IPC for each bar (e.g. click event might need to send a NATS request in a specific NATS context, only known by the process)
3. The hardcoded socket path is a problem in this case.
4. The process always knows its own socket path. The process should create the file /tmp/iconography.sock.{pid}
5. The click event should be something like '{exe} -sock /tmp/iconography.{pid} whatever-command'
