For building distributions, the user must extract Mario Kart Wii ISO/WBFS and contain original tracks due to LE-CODE specification issue.

For our goal purpose, it cannot require users to extract ISO. The best solution is containing track files into the program as array, but it has any problems due to the copyright issue. Needs to discuss whether track files can be stored as vector of unsigned 8-bit integer.

For the plan, the program will load the binary file that included all nintendo track files. It is used for building a distribution that contains nintendo tracks. The track files must be put in ``/tracks`` folder to compress into a file.
The compression will operate in build script and the compressed file will be stored in ``/res`` folder.
