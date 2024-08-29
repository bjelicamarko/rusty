import { Component } from '@angular/core';
import { AppService } from './app-service.service';
import { CustomReport } from './report';
import { Program } from './program';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'compiler-app';
  result: string = '';
  report: CustomReport = {
    diagnostics: [],
    symbol_table: []
  };
  program: Program = {
    code: `{ 
  let res = 0; 
  for (j = 0 to 10) { 
    res = res + j; 
  }
}`
  }

  constructor(private appService: AppService) { }

  submitProgram(): void {
    this.appService.getResult(this.program).subscribe(
      (response) => {
        this.report = response.body as CustomReport;
        console.log('Server response:', this.report);
        this.createResult();
      },
      (error) => {
        console.error('Error posting data', error);
      }
    );
  }

  createResult(): void {
    this.result = "";
    for (const pair of this.report.symbol_table) {
      this.result = this.result + pair.id + ": " + pair.value + " ";
    }
  }
}
