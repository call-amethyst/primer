import { SchemaPath, validate } from "@angular/forms/signals";

function passwordMatchValidator(path: SchemaPath<string>, comparePath: SchemaPath<string>, options?: { message?: string }): void {
    validate(path, ({ value, valueOf }) => {
      if (value() !== valueOf(comparePath)) {
        return { kind: 'passwordMismatch', message: options?.message || 'Passwords do not match' };
      }
      return null;
    });
  };

export default passwordMatchValidator;