import { Component, inject } from '@angular/core';
import { TableModule } from 'primeng/table';
import { TeacherService } from './teacher.service';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { SelectItem } from 'primeng/api';
import _ from 'lodash';

interface Teacher {
  id: number;
  payroll: number;
  name: string;
}

@Component({
  selector: 'app-teacher',
  standalone: true,
  imports: [TableModule, IconFieldModule, InputIconModule, InputTextModule],
  templateUrl: './teacher.component.html',
  styleUrl: './teacher.component.css',
})
export class TeacherComponent {
  teacherService: TeacherService = inject(TeacherService);
  totalRecords: number = 0;
  teachers: Teacher[] = [];
  modeOptions: SelectItem[] = [{ label: 'Equals', value: 'equals' }];
  emptyFilters: object = {
    id: { value: null, matchMode: 'startsWith' },
    payroll: { value: null, matchMode: 'startsWith' },
    name: { value: null, matchMode: 'startsWith' },
  };

  async loadTeachers(event: any) {
    console.log(event);
    if (
      _.isEqual(event.filters, this.emptyFilters) ||
      _.isEmpty(event.filters)
    ) {
      this.teachers = await this.teacherService.getTeachers(
        event.first,
        event.rows,
        event.sortField,
        event.sortOrder,
        event.filters
      );
      this.totalRecords = await this.teacherService.countTeachers();
    } else {
      this.teachers = await this.teacherService.getTeachers(
        event.first,
        event.rows,
        event.sortField,
        event.sortOrder,
        event.filters
      );
      this.totalRecords = this.teachers.length;
    }
  }
}
