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
import { TagModule } from "primeng/tag";
import { AccordionModule } from "primeng/accordion";
import { NgForOf } from "@angular/common";

@Component({
  selector: "app-student",
  standalone: true,
  imports: [
    TableModule,
    ButtonModule,
    RippleModule,
    TagModule,
    AccordionModule,
    NgForOf,
  ],
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
      .subscribe((grades) => {
        this.studentGrades = grades;
        console.log(grades);
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

  averageAnalyzer(grade: Grade): number[] {
    let currentAverage = 0;
    grade.firstGrade &&
      (currentAverage += (grade.firstGrade * grade.firstWeighing) / 100);
    grade.secondGrade &&
      (currentAverage += (grade.secondGrade * grade.secondWeighing) / 100);
    grade.thirdGrade &&
      (currentAverage += (grade.thirdGrade * grade.thirdWeighing) / 100);

    if (!grade.thirdGrade) {
      grade.thirdWeighing = 100 - grade.firstWeighing - grade.secondWeighing;
      if (currentAverage >= 70) {
        return [Number(currentAverage.toFixed(2)), 0];
      }
      let neededGrade = (70 - currentAverage) / (grade.thirdWeighing / 100);
      if (neededGrade > 100) {
        return [
          Number(currentAverage.toFixed(2)),
          Number(neededGrade.toFixed(2)),
        ];
      }
      return [
        Number(currentAverage.toFixed(2)),
        Number(neededGrade.toFixed(2)),
      ];
    }

    return [Number(currentAverage.toFixed(2))];
  }

  protected readonly Math = Math;
}
