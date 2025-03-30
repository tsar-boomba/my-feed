import { DateInputProps, DateInput as MantineDateInput } from '@mantine/dates';
import { Control, Controller, FieldValues, Path } from 'react-hook-form';

type Props<T extends FieldValues> = DateInputProps & {
	name: Path<T>;
	control: Control<T>;
};

export const DateInput = <T extends FieldValues>({
	control,
	name,
	...props
}: Props<T>) => {
	return (
		<Controller
			name={name}
			control={control}
			render={({ field }) => (
				<MantineDateInput
					{...props}
					onBlur={(e) => {
						props.onBlur?.(e);
						field.onBlur();
					}}
					onChange={(v) => {
						props.onChange?.(v);
						field.onChange(v);
					}}
				/>
			)}
		/>
	);
};
