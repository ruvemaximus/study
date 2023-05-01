export class SuccessResponse {
    constructor(message) {
        this.status='success';
        this.message=message;
    }
}

export class ErrorResponse {
    constructor(message) {
        this.status='error';
        this.message=message;
    }
}

