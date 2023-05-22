#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    if (argc <= 1) {
        printf("No command provided.\n");
        return 1;
    }

    size_t command_length = 0;
    for (int i = 1; i < argc; i++) {
        command_length += strlen(argv[i]) + 1;
    }

    char *command = (char *)malloc(command_length + 1);
    command[0] = '\0';

    for (int i = 1; i < argc; i++) {
        strcat(command, argv[i]);
        strcat(command, " ");
    }

    int ret = system(command);
    if (ret == -1) {
        printf("Command execution failed.\n");
        free(command);
        
        return 1;
    } else if (WIFEXITED(ret)) {
        int exit_status = WEXITSTATUS(ret);
        printf("Command exited with status: %d\n", exit_status);
    } else if (WIFSIGNALED(ret)) {
        int signal_number = WTERMSIG(ret);
        printf("Command terminated by signal: %d\n", signal_number);
    }

    free(command);

    return 0;
}
