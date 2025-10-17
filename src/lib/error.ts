
export class AppError extends Error{
    cause?: unknown;
    constructor(message: string,cause?: unknown){
        super(message);
        this.cause = cause;
    }
}

/** An error from the backend. */
export type ErrorResponse = {
    message: string
}