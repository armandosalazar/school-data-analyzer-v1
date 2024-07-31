import { Component, inject } from '@angular/core';
import { StudentService } from './student.service';

@Component({
  selector: 'app-student',
  standalone: true,
  imports: [],
  templateUrl: './student.component.html',
  styleUrl: './student.component.css',
})
export class StudentComponent {
  studentService: StudentService = inject(StudentService);

  getStudents(): void {
    this.studentService.getStudents().subscribe((students) => {
      console.log(students)
    });
  }
}
