typedef struct PositionOffset {
    long position_north;
    long position_east;
    long *std_dev_position_north; // OPTIONAL
    long *std_dev_position_east;  // OPTIONAL

    // ...
} PositionOffset_t;
