import {Pipe, PipeTransform} from '@angular/core';

@Pipe({
  name: 'timeFormatPipe'
})
export class TimeFormatPipe implements PipeTransform {
  transform(value: string): string {
    return value.slice(value.indexOf("T") + 1, value.lastIndexOf("."));
  }
}
