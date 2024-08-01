import { Component, inject } from "@angular/core";
import { StudentService, Student, Grade, Filter } from "./student.service";
import {
  TableModule,
  TableRowCollapseEvent,
  TableRowExpandEvent,
} from "primeng/table";
import _ from "lodash";
import { ButtonModule } from "primeng/button";
import { RippleModule } from "primeng/ripple";

@Component({
  selector: "app-student",
  standalone: true,
  imports: [TableModule, ButtonModule, RippleModule],
  templateUrl: "./student.component.html",
  styleUrl: "./student.component.css",
})
export class StudentComponent {
  studentService: StudentService = inject(StudentService);
  students: Student[] = [];
  totalRecords: number = 10;
  expandedRows = {};
  studentGrades: Grade[] = [];
  filters: Filter[] = [];

  loadStudents(event: any): void {
    console.log(JSON.stringify(event));
    if (_.isEqual(event.filters, {}) || _.isEmpty(event.filters)) {
      this.filters = [];
    } else {
      this.filters = Object.keys(event.filters).map((key) => {
        return {
          name: key,
          value: event.filters[key].value,
          matchMode: event.filters[key].matchMode,
        };
      });
    }

    console.log(this.filters);

    this.studentService
      .getStudents(event.rows, event.first, this.filters)
      .subscribe((students) => {
        this.students = students;
      });
    this.studentService.countStudents().subscribe((total) => {
      this.totalRecords = total;
      console.log(total);
    });
  }

  onRowExpand(event: TableRowExpandEvent) {
    console.log(event);
    this.studentService
      .getGradesByStudentId(event.data.id)
      .subscribe((data) => {
        this.studentGrades = data;
        console.log(data);
      });
  }

  onRowCollapse(event: TableRowCollapseEvent) {
    // this.messageService.add({
    //   severity: "success",
    //   summary: "Product Collapsed",
    //   detail: event.data.name,
    //   life: 3000,
    // });
  }
}
